import datetime
import logging
from pathlib import Path

import cc1101

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s:%(levelname)s:%(name)s:%(funcName)s:%(message)s",
    datefmt="%Y-%m-%dT%H:%M:%S%z"
)

SUPPORTED_BAUD = [
    "600",
"1200",
    "2400",
"4800",
  "9600",
 "14400",
 "19200",
  "28800",
  "38400",
  "57600",
  "76800",
  "115200",
  "250000",
]

p = Path() / "data.csv"

with p.open("a") as f:

    with cc1101.CC1101() as transceiver:
        transceiver.set_base_frequency_hertz(433.92e6)
        for baud in SUPPORTED_BAUD:
            transceiver.set_symbol_rate_baud(baud)
            for sync_mode in [e for e in cc1101.SyncMode]:
                transceiver.set_sync_mode(sync_mode, True)
                print(transceiver)
                #transceiver.transmit(b"\x01\xff\x00 message")
                while True:
                    start_time = datetime.datetime.now()
                    packet = transceiver._wait_for_packet(timeout_seconds=10,gdo0_gpio_line_name="GPIO24".encode())
                    if packet:
                        end = datetime.datetime.now()
                        d = (end - start_time).total_seconds()
                        s = f"{packet.rssi_dbm},{packet.checksum_valid},{packet.link_quality_indicator},{packet.payload.hex()},{d},{sync_mode},{baud}\n"
                        print(s)
                        f.write(s)
