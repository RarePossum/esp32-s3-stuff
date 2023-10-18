## Part Pin Plan. 

### LEFT
GPIO43 44 - UART1
GPIO0 - FLASH

45 Pins to use

6 Output 26 29 30 40 47 48
3 Input (Low Impedence) 39 41 42

### RIGHT

18 19 20 -> USB
43 44 -> UART1
0 -> Flash
39 40 -> UART2
40 Pins to use 
4 Output 26 29 39 47 48
2 Input Low Impedence 41 42 


### NUM
GPIO43 44 - UART2
GPIO0 - FLASH

45 Pins to use

6 Output 26 29 30 40 47 48
3 Input (Low Impedence) 39 41 42

## Rough wiring
Right has USB conenction. VBUS delivers power to RIGHT, before its carried via 4-pins (PJ320A TRSS connector and TRSS Cables) to the other boards. These 4pins also serve as UART communication from LEFT and NUM to RIGHT

At RIGHT, the UART signals are read, and then a packet is made from the signals. 