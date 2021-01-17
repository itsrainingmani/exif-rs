# EXIF in Rust

This is a Rust library for reading/writing EXIF data in Rust

## What is EXIF

Exif stands for Exchangeable Image file Format. It is a standard that specifies the formats for images, sound and ancillary tags used by digital cameras, scanners and other systems handling image and sound files recorded by digital cameras.

The spec uses JPEG, TIFF and WAV file formats with the addition of specific metadata tags. Exif Data are embedded within the image file itself.

There are many existing software libs for parsing Exif data from files & to read/write Exif tag values like _libexif_, _PIL/Pillow_ and _Exiv2_.

The Exif format also has standard tags for location information. Many cameras and smartphones automatically stores the location information in the Exif header when a picture is taken.

## EXIF Metadata

There are many metadata tags defined in the Exif standard -

- Date and Time info
- Camera settings: Make and Model, Orientation(rotation), ISO, aperture, Shutter Speed, Focal Length, Metering Mode, resolution, color space etc
- Thumbnail for previewing the picture
- Descriptions
- Copyright info

## EXIF Structure

Every JPEG file starts from binary value '0xFFD8', ends with binary value '0xFFD9'. There are several binary 0xFFXX data in JPEG data, they are called as "Marker", and it means the period of JPEG information data. 0xFFD8 means SOI(Start of image), 0xFFD9 means EOI(End of image).

Basically, EXIF file format is the same as JPEG file format. Exif inserts some of image/digicam information data and thumbnail image to JPEG in conformity to JPEG specification.

Example of a JPEG File -

| SOI Marker | Marker XX size=SSSS | Marker YY size=TTTT | SOS Marker size=UUUU | Image stream | EOI Marker |
| ---------- | ------------------- | ------------------- | -------------------- | ------------ | ---------- |
| FFD8       | FFXX SSSS DDDD..... | FFYY TTTT DDDD..... | FFDA UUUU DDDD.....  | I I I I....  | FFD9       |

Example of an EXIF File -

| SOI Marker | APP1 Marker | APP1 Data                    | Other Marker         |
| ---------- | ----------- | ---------------------------- | -------------------- |
| FFD8       | FFE1        | SSSS 457869660000 TTTT...... | FFXX SSSS DDDD...... |
