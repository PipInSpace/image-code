# About

image-code can be used to encode and decode data into images without disturbing the original image content (too much).
The output image stores endoded data in the R-channel pixels, G- and B-channels point to the next pixel in the data sequence.
image-code encodes this data as a difference from the average of the 4 surrounding pixels, so pixels usually remain close to their original colour in pictures (but disturbance may be visible in very flat or artificial images).

# Usage

To configure image-code, you may change the `STARTING_POS` and `BITS_PER_PX` constants (IMPORTANT! These must be the same at encoding and decoding).

- `STARTING_POS` - The coordinate of the first pixel in the data sequence
- `BITS_PER_PX` - The amount of bits stored/changed per pixel. Ranges from 1-7. Bigger values result in bigger disturbance but fewer changed pixels.

To run the program use the following commands:

    cargo run "source_image" "data" -e  (Encodes "data" into "source_image")
    cargo run "endoded_image" -d        (Decodes "encoded_image" into encoded_image.dat)

Replace the strings with the path to your files.

# Example

This README file was encoded into the following image (`STARTING_POS`: (4, 4), `BITS_PER_PX`: 4). Shown is the base image, the image with encoded data and the data noise (amplified)

<a><img src="https://github.com/PipInSpace/image-code/blob/main/examples/ion.png?raw=true" width="33.3333%"></a><a><img src="https://github.com/PipInSpace/image-code/blob/main/examples/ion_enc.png?raw=true" width="33.3333%"></a><a><img src="https://github.com/PipInSpace/image-code/blob/main/examples/ion_data_noise_amplified.png?raw=true" width="33.3333%"></a>
<img src="https://github.com/PipInSpace/image-code/blob/main/examples/ion.png?raw=true" width="100%">

The base image<br><br>

<img src="https://github.com/PipInSpace/image-code/blob/main/examples/ion_enc.png?raw=true" width="100%">

The image with the encoded data<br><br>

<img src="https://github.com/PipInSpace/image-code/blob/main/examples/ion_data_noise_amplified.png?raw=true" width="100%">

The data pixels extracted and brightened<br><br>

Image source: https://commons.wikimedia.org/wiki/File:Ion_Engine_Test_Firing_-_GPN-2000-000482.jpg
