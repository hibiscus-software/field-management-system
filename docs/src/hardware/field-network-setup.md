# Field Network Setup

## Ethernet Cabling

Run one Ethernet cable from the field control cabinet (FCC) to each station control cabinet (SCC).
The FCC end of the cables should be plugged in to either `RED SCC TRUNK` or `BLUE SCC TRUNK` and
be plugged into the respective SCC. The port on the SCCs is labeled as `FCC UPLINK`.

Run one Ethernet cable from the FCC port labeled `FIELD ROUTER` to the field router and another Ethernet
cable from the FCC port labeled `FIELD SERVER` to the field server.

Run one Ethernet cable from the SCCs to the alliance stations for use with team's driver stations. The
ports on the SCC are labeled as `STATION 1`, `STATION 2`, and `STATION 3`. The cables should be set down
at the respective alliancestations on either side.

## Sensor Cabling

Run the sensor cable labeled `FIELD E-STOP` to the field e-stop and plug it into the FCC port labeled
`FIELD E-STOP`. Run the sensor cable labeled `FIELD STACK LIGHT` to the field stack light and plug it
into the FCC port labeled `FIELD STACK LIGHT`.

Run the sensor cables from the FCC ports labeled `RED E-STOP 1`, `RED E-STOP 2`, and `RED E-STOP 3` to
the respective e-stops at the alliance stations. Do the same for the stack lights, run the sensor cables
from the FCC ports labeled `RED STACK LIGHT 1`, `RED STACK LIGHT 2`, `RED STACK LIGHT 3` to the respective
stack lights at the alliance stations. Run the same sensor cables labeled for the blue alliance station
side respectively.

## Power Cabling

Run a IEC power cable to the FCC and to each of the SCCs and plug them into the port labeled
`MAIN POWER IN`. Run a IEC power cable from the FCC port labeled `AUX POWER OUT` to the auxiliary
control cabinet (ACC) if it is being used.