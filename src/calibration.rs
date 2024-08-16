// This should be the *only* file that interfaces with the burn library.

use burn::data::dataloader::Dataset;

const LOOKBEHIND: usize = 10;

// The front end API
#[derive(Clone, Default, Debug)]
pub struct CalibController {
    pub dataset: PsyLinkDataset,
}

impl CalibController {
    pub fn add_packet(&mut self, sample: Vec<u8>) {
        self.dataset.all_packets.push(sample);
    }

    pub fn add_datapoint(&mut self, datapoint: Datapoint) {
        self.dataset.datapoints.push(datapoint);
    }

    // When you use this method, make sure to add the packet first.
    pub fn get_current_index(&self) -> usize {
        return self.dataset.all_packets.len();
    }
}

// This is a slim variant of a TrainingSample. It's faster to work with, but can't be
// used to train a NN directly. It's only valid in the context of a PsyLinkDataset,
// and PsyLinkDataset.get() will turn it into a TrainingSample when needed.
#[derive(Clone, Default, Debug)]
pub struct Datapoint {
    pub packet_index: usize,
    pub label: u8,
}

// This is a pair of features+labels that will be used for training the NN.
// It has a one-to-one mapping to a Datapoint struct.
#[derive(Clone, Default, Debug)]
pub struct TrainingSample {
    pub features: Vec<Vec<u8>>,
    pub label: u8,
}

// The dataset contains a list of all received packets in this session,
// along with datapoints which were recorded when the user was asked to
// perform a particular movement.
#[derive(Clone, Default, Debug)]
pub struct PsyLinkDataset {
    pub datapoints: Vec<Datapoint>,
    pub all_packets: Vec<Vec<u8>>,
}

impl Dataset<TrainingSample> for PsyLinkDataset {
    // Constructs a TrainingSample with training features that include
    // the signals at the time of recording, along with some amount of
    // signals from the past.
    fn get(&self, index: usize) -> Option<TrainingSample> {
        let datapoint = self.datapoints.get(index)?;

        if datapoint.packet_index <= LOOKBEHIND {
            return None;
        }
        let start = datapoint.packet_index - LOOKBEHIND;
        let end = datapoint.packet_index;
        let packet = self.all_packets.get(start..end)?;

        Some(TrainingSample {
            features: (*packet).iter().cloned().collect(),
            label: datapoint.label,
        })
    }
    fn len(&self) -> usize {
        return self.datapoints.len();
    }
}
