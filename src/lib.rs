use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use easy_ml::differentiation::{Record, WengertList};
use easy_ml::linear_algebra;
use easy_ml::matrices::Matrix;
use easy_ml::numeric::extra::Real;
use easy_ml::numeric::Numeric;

use std::cmp;
use std::convert::TryFrom;
use std::convert::TryInto;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Use `wee_alloc` as the global
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = "logProgress")]
    fn logProgress(percent: f64);
    #[wasm_bindgen(js_namespace = globalThis, js_name = "logBatchLoss")]
    fn logBatchLoss(percent: f64);
    #[wasm_bindgen(js_namespace = globalThis, js_name = "getNetworkWeights")]
    fn getNetworkWeights() -> String;
}

/**
 * Wraps the JavaScript function in a snake_case name
 */
fn log_progress(percent: f64) {
    logProgress(percent);
}

fn log_batch_loss(percent: f64) {
    logBatchLoss(percent);
}

fn get_network_weights() -> String {
    return getNetworkWeights();
}

const WIDTH: usize = 16;
const HEIGHT: usize = 16;
// const SAMPLES: usize = 6483;
const TRAINING_SIZE: usize = 5186;
const TESTING_SIZE: usize = 1297;
const LEARNING_RATE: f64 = 0.32;
const LEARNING_RATE_DISCOUNT_FACTOR: f64 = 0.96875;

/// mbti data is grayscale 0-1 range
type Pixel = f64;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    data: Vec<Pixel>,
}

#[wasm_bindgen]
impl Image {
    /// Creates a new Image
    pub fn new() -> Image {
        Image {
            data: Vec::with_capacity(WIDTH * HEIGHT),
        }
    }

    /// Accesses the data buffer of this Image, for JavaScript to fill with the actual data
    pub fn buffer(&mut self) -> *const Pixel {
        self.data.as_ptr()
    }

    pub fn set_length(&mut self) {
        // this is safe because we will only call it after initialising all elements
        // via buffer access on the JS side
        unsafe {
            self.data.set_len(WIDTH * HEIGHT);
        }
    }
}

impl From<Image> for Matrix<f64> {
    fn from(image: Image) -> Self {
        Matrix::from_flat_row_major((1, WIDTH * HEIGHT), image.data).map(|pixel| pixel)
    }
}

/// A label type for the MBTI personality types consisting of 8 possible letters
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBType {
    I = 0b10000000,
    E = 0b01000000,
    S = 0b00100000,
    N = 0b00010000,
    T = 0b00001000,
    F = 0b00000100,
    J = 0b00000010,
    P = 0b00000001,
}

// A label for the MBTI personality types consisting of 16 possible classifiers
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBTI {
    ENFP = 0b01010101,  // "ENFP",
    ENFJ = 0b01010110,  // "ENFJ",
    ENTP = 0b01011001,  // "ENTP",
    ENTJ = 0b01011010,  // "ENTJ",
    ESFP = 0b01100101,  // "ESFP",
    ESFJ = 0b01100110,  // "ESFJ",
    ESTP = 0b01101001,  // "ESTP",
    ESTJ = 0b01101010,  // "ESTJ",
    INFP = 0b10010101, // "INFP",
    INFJ = 0b10010110, // "INFJ",
    INTP = 0b10011001, // "INTP",
    INTJ = 0b10011010, // "INTJ",
    ISFP = 0b10100101,  // "ISFP",
    ISFJ = 0b10100110,  // "ISFJ",
    ISTP = 0b10101001, // "ISTP",
    ISTJ = 0b10101010, // "ISTJ",
}

impl TryFrom<u8> for MBTI {
    type Error = &'static str;

    fn try_from(integer: u8) -> Result<Self, Self::Error> {
        match integer {
            0b01010101 => Ok(MBTI::ENFP),
            0b01010110 => Ok(MBTI::ENFJ),
            0b01011001 => Ok(MBTI::ENTP),
            0b01011010 => Ok(MBTI::ENTJ),
            0b01100101 => Ok(MBTI::ESFP),
            0b01100110 => Ok(MBTI::ESFJ),
            0b01101001 => Ok(MBTI::ESTP),
            0b01101010 => Ok(MBTI::ESTJ),
            0b10010101 => Ok(MBTI::INFP),
            0b10010110 => Ok(MBTI::INFJ),
            0b10011001 => Ok(MBTI::INTP),
            0b10011010 => Ok(MBTI::INTJ),
            0b10100101 => Ok(MBTI::ISFP),
            0b10100110 => Ok(MBTI::ISFJ),
            0b10101001 => Ok(MBTI::ISTP),
            0b10101010 => Ok(MBTI::ISTJ),
            _ => Err("Invalid MBTI type"),
        }
    }
}

impl From<&str> for MBTI {
    fn from(label: &str) -> Self {
        match label {
            "ENFP" => MBTI::ENFP,
            "ENFJ" => MBTI::ENFJ,
            "ENTP" => MBTI::ENTP,
            "ENTJ" => MBTI::ENTJ,
            "ESFP" => MBTI::ESFP,
            "ESFJ" => MBTI::ESFJ,
            "ESTP" => MBTI::ESTP,
            "ESTJ" => MBTI::ESTJ,
            "INFP" => MBTI::INFP,
            "INFJ" => MBTI::INFJ,
            "INTP" => MBTI::INTP,
            "INTJ" => MBTI::INTJ,
            "ISFP" => MBTI::ISFP,
            "ISFJ" => MBTI::ISFJ,
            "ISTP" => MBTI::ISTP,
            "ISTJ" => MBTI::ISTJ,
            _ => panic!("Invalid MBTI type"),
        }
    }
}

impl From<usize> for MBTI {
    fn from(label: usize) -> Self {
        match label {
            0 => MBTI::ENFP,
            1 => MBTI::ENFJ,
            2 => MBTI::ENTP,
            3 => MBTI::ENTJ,
            4 => MBTI::ESFP,
            5 => MBTI::ESFJ,
            6 => MBTI::ESTP,
            7 => MBTI::ESTJ,
            8 => MBTI::INFP,
            9 => MBTI::INFJ,
            10 => MBTI::INTP,
            11 => MBTI::INTJ,
            12 => MBTI::ISFP,
            13 => MBTI::ISFJ,
            14 => MBTI::ISTP,
            15 => MBTI::ISTJ,
            _ => panic!("Invalid MBTI type"),
        }
    }
}

impl From<MBTI> for usize {
    fn from(label: MBTI) -> Self {
        match label {
            MBTI::ENFP => 0,
            MBTI::ENFJ => 1,
            MBTI::ENTP => 2,
            MBTI::ENTJ => 3,
            MBTI::ESFP => 4,
            MBTI::ESFJ => 5,
            MBTI::ESTP => 6,
            MBTI::ESTJ => 7,
            MBTI::INFP => 8,
            MBTI::INFJ => 9,
            MBTI::INTP => 10,
            MBTI::INTJ => 11,
            MBTI::ISFP => 12,
            MBTI::ISFJ => 13,
            MBTI::ISTP => 14,
            MBTI::ISTJ => 15,
        }
    }
}

impl From<MBTI> for &str {
    fn from(label: MBTI) -> Self {
        match label {
            MBTI::ENFP => "ENFP",
            MBTI::ENFJ => "ENFJ",
            MBTI::ENTP => "ENTP",
            MBTI::ENTJ => "ENTJ",
            MBTI::ESFP => "ESFP",
            MBTI::ESFJ => "ESFJ",
            MBTI::ESTP => "ESTP",
            MBTI::ESTJ => "ESTJ",
            MBTI::INFP => "INFP",
            MBTI::INFJ => "INFJ",
            MBTI::INTP => "INTP",
            MBTI::INTJ => "INTJ",
            MBTI::ISFP => "ISFP",
            MBTI::ISFJ => "ISFJ",
            MBTI::ISTP => "ISTP",
            MBTI::ISTJ => "ISTJ",
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Dataset {
    images: Vec<Image>,
    labels: Vec<MBTI>,
}

#[wasm_bindgen]
impl Dataset {
    pub fn new_training() -> Dataset {
        Dataset {
            images: Vec::with_capacity(TRAINING_SIZE),
            labels: Vec::with_capacity(TRAINING_SIZE),
        }
    }

    pub fn new_testing() -> Dataset {
        Dataset {
            images: Vec::with_capacity(TESTING_SIZE),
            labels: Vec::with_capacity(TESTING_SIZE),
        }
    }

    pub fn add(&mut self, image: Image, label: usize) {
        self.images.push(image);
        self.labels.push(label.try_into().expect("Label invalid"));
    }
}

/// A neural network configuration to classify the mbti data
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuralNetwork {
    weights: Vec<Matrix<f64>>,
    epochs: i32,
    //buffer: Vec<f64>,
}

const FIRST_HIDDEN_LAYER_SIZE: usize = 128;
const SECOND_HIDDEN_LAYER_SIZE: usize = 64;
const OUTPUT_LAYER_SIZE: usize = 16;

// fn relu<T: Numeric + Copy>(x: T) -> T {
//     if x > T::zero() {
//         x
//     } else {
//         T::zero()
//     }
// }

fn sigmoid<T: Numeric + Real + Copy>(x: T) -> T {
    T::one() / (T::one() + (-x).exp())
}

#[wasm_bindgen]
impl NeuralNetwork {
    /// Creates a new Neural Network configuration of randomised weights
    /// and a simple feed forward architecture.
    pub fn new() -> NeuralNetwork {
        let weights: String = get_network_weights();

        if weights.as_str() != "" {
            let network = NeuralNetwork::from_json(&weights.as_str());
            if network.epochs > 0 {
                return network;
            }
        }
        else {
            log!("No weights found");
        }
        let mut weights = vec![
            Matrix::empty(0.0, (WIDTH * HEIGHT, FIRST_HIDDEN_LAYER_SIZE)),
            Matrix::empty(0.0, (FIRST_HIDDEN_LAYER_SIZE, SECOND_HIDDEN_LAYER_SIZE)),
            Matrix::empty(0.0, (SECOND_HIDDEN_LAYER_SIZE, OUTPUT_LAYER_SIZE)),
        ];
        for i in 0..weights.len() {
            for j in 0..weights[i].size().0 {
                for k in 0..weights[i].size().1 {
                    weights[i].set(j, k, (2.0 * js_sys::Math::random()) - 1.0);
                }
            }
        }
        NeuralNetwork {
            weights,
            epochs: 0, //buffer: Vec::with_capacity(0),
        }
    }

    pub fn layers(&self) -> usize {
        self.weights.len()
    }

    pub fn classify(&self, image: &Image) -> MBTI {
        let input: Matrix<f64> = image.clone().into();
        // this neural network is a simple feed forward architecture, so dot product
        // the input through the network weights and apply the sigmoid activation
        // function each step, then take softmax to produce an output
        let output = {
            let layer1 = (input * &self.weights[0]).map(sigmoid);
            let layer2 = (layer1 * &self.weights[1]).map(sigmoid);
            layer2 * &self.weights[2]
        };
        let classification = linear_algebra::softmax(output.row_major_iter());
        // find the index of the largest softmax'd label
        classification
            .iter()
            // find argmax of the output
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("NaN should not be in list"))
            // convert from usize into a MBTI, by construction classification only has
            // 16 elements, so the index will fit into a MBTI
            .map(|(i, _)| i as usize)
            .unwrap()
            .try_into()
            .unwrap()
    }

    /// Trains the neural net for 1 epoch and returns the average loss on the epoch
    pub fn train(&mut self, training_data: &Dataset) -> f64 {
        log_progress(0.0);
        let history = WengertList::new();
        let mut training = NeuralNetworkTraining::from(&self, &history, self.epochs);
        let loss = training.train_epoch(training_data, &history);
        training.update(self);
        log_progress(1.0);
        self.epochs += 1;
        self.to_storage();
        loss
    }

    /// Computes the accuracy on a dataset and returns the percent correctly classified
    /// as a number between 0 and 1.
    pub fn accuracy(&self, dataset: &Dataset) -> f64 {
        let mut correct = 0;
        for i in 0..dataset.images.len() {
            let prediction = self.classify(&dataset.images[i]);
            if prediction == dataset.labels[i] {
                correct += 1;
            }
        }
        (correct as f64) / (dataset.images.len() as f64)
    }

    /// Serialises the neural network to a JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialise neural network")
    }

    /// Deserialises a neural network from a JSON string
    pub fn from_json(json: &str) -> NeuralNetwork {
        // log!("{}", &json);
        serde_json::from_str(json)
            .map_err(|e| JsValue::from_str(&e.to_string()))
            .unwrap()
    }

    /// This is used to save the neural network to local storage
    pub fn to_storage(&self) -> () {
        let json = self.to_json();
        log!("{}", json);
        // TODO: Integration with imutable blockchain storage.
    }

    /// This is used to load the neural network from local storage
    pub fn from_storage(json: &str) -> NeuralNetwork {
        // TODO: Make this async the weights are loaded from the blockchain.
        NeuralNetwork::from_json(&json)
    }
}

/// At the time of writing, #[wasm_bindgen] does not support lifetimes or type
/// parameters. The Record trait has a lifetime parameter because it must not
/// outlive its WengertList. Unfortunately at the time of writing the WengertList
/// constructor also cannot be a constant function because type parameters other than
/// Sized are not stabalised. Additionally, the WengertList does not implement Sync
/// so it cannot be a shared static variable. The cummulative effect of these restrictions
/// mean that I cannot find a way to pass any structs to JavaScript which include a Record
/// type, even though thread safety is a non concern and any such struct that would be
/// passed to JavaScript would also have been defined to own the WengertList that the Records
/// referenced - ie, such a struct would be completely safe, but I can't find a way to
/// get the Rust type system to agree.
///
/// If you're reading this and #[wasm_bindgen] has added lifetime support, or it's
/// possible to make a WengertList with a &static lifetime, or there's a way to create
/// a struct which owns the WengertList and Records but does not bubble the useless lifetime
/// up then please open an issue or pull request to let me know.
///
/// Until then we will have to not share such types with JavaScript. This is actually
/// not a huge issue, because Records are only needed for training anyway.
#[derive(Clone, Debug)]
struct NeuralNetworkTraining<'a> {
    weights: Vec<Matrix<Record<'a, f64>>>,
    learning_rate: f64,
}

const BATCH_SIZE: usize = 32;

impl<'a> NeuralNetworkTraining<'a> {
    /// Given a WengertList which will be used exclusively for training this struct,
    /// and an existing configuration for weights, creates a new NeuralNetworkTraining
    fn from(
        configuration: &NeuralNetwork,
        history: &'a WengertList<f64>,
        epochs: i32,
    ) -> NeuralNetworkTraining<'a> {
        let mut weights = Vec::with_capacity(configuration.weights.len());
        for i in 0..configuration.weights.len() {
            weights.push(Matrix::empty(
                Record::variable(0.0, &history),
                configuration.weights[i].size(),
            ));
            for j in 0..configuration.weights[i].size().0 {
                for k in 0..configuration.weights[i].size().1 {
                    let neuron = configuration.weights[i].get(j, k);
                    weights[i].set(j, k, Record::variable(neuron, &history));
                }
            }
        }
        NeuralNetworkTraining {
            weights,
            learning_rate: LEARNING_RATE * LEARNING_RATE_DISCOUNT_FACTOR.powi(epochs),
        }
    }

    /// Updates an existing neural network configuration to the new weights
    /// learned through training.
    fn update(&self, configuration: &mut NeuralNetwork) {
        for i in 0..self.weights.len() {
            for j in 0..self.weights[i].size().0 {
                for k in 0..self.weights[i].size().1 {
                    let neuron = self.weights[i].get(j, k).number;
                    configuration.weights[i].set(j, k, neuron);
                }
            }
        }
    }

    /// Classification is very similar for training, except we stay in floating point
    /// land so we can backprop the error.
    /// This function takes an iterator of Images and MBTIs, and updates the weights
    /// after getting the errors on the entire batch, returning the average loss for
    /// the batch.
    pub fn train<I>(&mut self, batch: I, learning_rate: f64, history: &'a WengertList<f64>) -> f64
    where
        I: Iterator<Item = (&'a Image, MBTI)>,
    {
        let mut errors = Vec::with_capacity(BATCH_SIZE);
        for (image, label) in batch {
            let input: Matrix<f64> = image.clone().into();
            // this neural network is a simple feed forward architecture, so dot product
            // the input through the network weights and apply the sigmoid activation
            // function each step, then take softmax to produce an output
            let output = {
                let i = input.map(|p| Record::constant(p));
                let layer1 = (i * &self.weights[0]).map(sigmoid);
                let layer2 = (layer1 * &self.weights[1]).map(sigmoid);
                layer2 * &self.weights[2]
            };
            let classification = linear_algebra::softmax(output.row_major_iter());
            //let classification = NeuralNetworkTraining::softmax(output.row_major_iter());
            // Get what we predicted for the true label. To minimise error, we should
            // have predicted 1
            let prediction: Record<f64> = classification[Into::<usize>::into(label)];
            // If we predicted 1 for the true label, error is 0, likewise, if
            // we predicted 0 for the true label, error is 1.
            let error: Record<f64> = Record::constant(1.0) - prediction;
            errors.push(error);
        }
        let batch_size = errors.len();
        let error: Record<f64> = errors.drain(..).sum();
        let derivatives = error.derivatives();
        // update weights to minimise error, note that if error was 0 this
        // trivially does nothing
        self.weights[0].map_mut(|x| x - (derivatives[&x] * learning_rate));
        self.weights[1].map_mut(|x| x - (derivatives[&x] * learning_rate));
        self.weights[2].map_mut(|x| x - (derivatives[&x] * learning_rate));
        // reset gradients
        history.clear();
        self.weights[0].map_mut(Record::do_reset);
        self.weights[1].map_mut(Record::do_reset);
        self.weights[2].map_mut(Record::do_reset);
        error.number / (batch_size as f64)
    }

    /// Performs minibatch SGD for one epoch on all of the training data in a random order,
    /// returning the average loss for the entire epoch.
    pub fn train_epoch(
        &mut self,
        training_data: &'a Dataset,
        history: &'a WengertList<f64>,
    ) -> f64 {
        let random_numbers = EndlessRandomGenerator {};
        let random_index_order: Vec<usize> = {
            let mut indexes: Vec<(usize, f64)> = (0..training_data.images.len())
                .zip(random_numbers)
                .collect();
            // sort by the random numbers we zipped
            indexes.sort_by(|(_, i), (_, j)| i.partial_cmp(j).unwrap());
            // drop the random numbers in the now randomised list of indexes
            indexes.drain(..).map(|(x, _)| x).collect()
        };
        let mut epoch_losses = 0.0;
        let mut batch_losses = 0.0;
        let mut progress = 0;
        let mut i = 0;
        loop {
            // compute the start and end indexes which will slice the random_index_order vec
            // to obtain a slice of indexes into the training data. Until reaching the end
            // of the datsset this will always be BATCH_SIZE, but may be smaller on the final
            // one.
            let start = i;
            let end = cmp::min(random_index_order.len(), start + BATCH_SIZE);
            let batch_indexes = &random_index_order[start..end];
            if progress % 5 == 0 {
                log_progress(i as f64 / (training_data.images.len() as f64));
            }
            // create a batch of tuples of referenced images and corresponding labels
            let batch = batch_indexes
                .iter()
                .map(|&index| (&training_data.images[index], training_data.labels[index]));
            let loss = self.train(batch, self.learning_rate, history);
            epoch_losses += loss;
            batch_losses += loss;
            // Report progress to the Web Worker after every 100 images (5 batches
            // for a BATCH_SIZE of 20).
            if progress % 5 == 0 && progress != 0 {
                if progress == 5 {
                    // 1 additional batch of images is summed in the first progress
                    // report because we don't report the loss on the first batch
                    // even though 0 % 5 == 0, so divide by 6 to get average loss
                    log_batch_loss(batch_losses / 6.0);
                } else {
                    log_batch_loss(batch_losses / 5.0);
                }
                batch_losses = 0.0;
            }
            progress += 1;
            if end == random_index_order.len() {
                break;
            }
            i += BATCH_SIZE;
        }
        epoch_losses / (training_data.images.len() as f64 / BATCH_SIZE as f64)
    }
}

struct EndlessRandomGenerator {}

impl Iterator for EndlessRandomGenerator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        // always return Some, hence this iterator is infinite
        Some(js_sys::Math::random())
    }
}
