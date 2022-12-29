import datetime
import logging
from pathlib import Path

import cc1101

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s:%(levelname)s:%(name)s:%(funcName)s:%(message)s",
    datefmt="%Y-%m-%dT%H:%M:%S%z",
)
# Original baud bands, nothing in the higher freqs was found
# SUPPORTED_BAUD = [
# 600,
# 1200,
# 2400,
# 4800,
# 9600,
# 14400,
# 19200,
# 28800,
# 38400,
# 57600,
# 76800,
# 115200,
# 250000,
# ]

# NEW_BANDS = [50, 110, 300, 600, 1200, 2400, 4800]
NEW_BANDS = [4800]

SEARCH_TIMEOUT = 2

p = Path() / "data.csv"

need_header = not (p.exists() and p.read_text().startswith("dbm"))

with p.open("a") as f:

    if need_header:
        header = f"dbm,checksum,linkQualityIndicator," + f"duration,syncMode,baud," + f"mancEncoding,payload\n"
        f.write(header)

    while True:
        for manc_encoding in [True, False]:
            for baud in NEW_BANDS:
                for sync_mode in [e for e in cc1101.SyncMode]:
                    with cc1101.CC1101() as transceiver:
                        if manc_encoding:
                            transceiver.enable_manchester_code()
                        transceiver.set_base_frequency_hertz(433.92e6)
                        transceiver.set_symbol_rate_baud(baud)
                        transceiver.set_packet_length_bytes(1024)
                        # for sync_mode in [e for e in cc1101.SyncMode]:
                        transceiver.set_sync_mode(sync_mode)
                        print(transceiver)
                        # transceiver.transmit(b"\x01\xff\x00 message")
                        start_time = datetime.datetime.now()
                        packet = transceiver._wait_for_packet(
                            timeout_seconds=SEARCH_TIMEOUT, gdo0_gpio_line_name="GPIO24".encode()
                        )
                        if packet:
                            end = datetime.datetime.now()
                            d = (end - start_time).total_seconds()
                            s = (
                                f"{packet.rssi_dbm},{packet.checksum_valid},{packet.link_quality_indicator}," + f"{d},{sync_mode},{baud}," + f"{manc_encoding},{packet.payload.hex()}\n"
                            )
                            print(s)
                            f.write(s)
