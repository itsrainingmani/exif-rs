# EXIF in Rust

This is a Rust library for reading/writing EXIF data in Rust

## What is EXIF

Exif stands for Exchangeable Image file Format. It is a standard that specifies the formats for images, sound and ancillary tags used by digital cameras, scanners and other systems handling image and sound files recorded by digital cameras.

The spec uses JPEG, TIFF and WAV file formats with the addition of specific metadata tags. Exif Data are embedded within the iamge file itself.

There are many existing software libs for parsing Exif data from files & to read/write Exif tag values like _libexif_, _PIL/Pillow_ and _Exiv2_.

The Exif format also has standard tags for location information. Many cameras and smartphones automatically stores the location information in the Exif header when a picture is taken.

## EXIF Metadata

There are many metadata tags defined in the Exif standard -

- Date and Time info
- Camera settings: Make and Model, Orientation(rotation), ISO, aperture, Shutter Speed, Focal Length, Metering Mode, resolution, color space etc
- Thumbnail for previewing the picture
- Descriptions
- Copyright info
