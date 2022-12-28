import logging
from pathlib import Path

import cc1101

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s:%(levelname)s:%(name)s:%(funcName)s:%(message)s",
    datefmt="%Y-%m-%dT%H:%M:%S%z"
)

p = Path() / "data.csv"

with p.open("a") as f:

    with cc1101.CC1101() as transceiver:
        transceiver.set_base_frequency_hertz(433.92e6)
        print(transceiver)
        #transceiver.transmit(b"\x01\xff\x00 message")
        while True:
            packet = transceiver._wait_for_packet(timeout_seconds=10,gdo0_gpio_line_name="GPIO24".encode())
            if packet:
                s = f"{packet.rssi_dbm},{packet.checksum_valid},{packet.link_quality_indicator},{packet.payload.hex()}\n"
                print(s)
                f.write(s)
