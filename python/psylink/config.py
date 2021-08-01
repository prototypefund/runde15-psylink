DEFAULT_RUN_NAME = 'default'
DEFAULT_BLE_ADDRESS = '25:68:C7:F3:BC:6B'
SAMPLE_RATE = 500  # Hz
SIGNAL_BUFFER_SIZE = 2000
ASSUMED_BLE_LATENCY = 0.1  # seconds
RECORD_EVERY_N_SAMPLES = 1
REDRAW_SIGNALS_DELAY = 100  # milliseconds

# AI Constants
SEED_TENSORFLOW = SEED_NUMPY = 1337
DEFAULT_TRAINING_EPOCHS = 25
DEFAULT_CHANNELS = 1
IMU_CHANNELS = 6  # For gyroscope and accelerometer
FEATURE_BUFFER_SIZE = 2**20
FEATURE_WINDOW_SIZE = int(SAMPLE_RATE / 2)  # Enough samples to fit half a second
LABEL_SEPARATOR = ','
BATCH_SIZE = 64
TRAIN_SPLIT = 0.8
