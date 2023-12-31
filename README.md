# Ray Tracer
A small ray tracer implemented in rust for the purpose of learning Rust (and ray tracing!) using [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## The Journey
### Testing out the PPM output in Rust.
<img width="50%" src="./renderings/test_image.png" style="aspect-ratio: 16/9">


### First ray traced circle.
<img width="100%" src="./renderings/test_image_5-2.png" style="aspect-ratio: 16/9">

### Mapping surface norms to R/G/B. More volume-y now.
<img width="100%" src="./renderings/test_image_6-2.png" style="aspect-ratio: 16/9">

### Added Antialiasing.
<img width="100%" src="./renderings/test_image_7-2_release.png" style="aspect-ratio: 16/9">

### Building a word with a list of objects.
<img width="100%" src="./renderings/test_image_7-5.png" style="aspect-ratio: 16/9">

### Initial rendering of diffuse material using 2 ray bounces, very prominent shadow acne.
Diffuse material scatters light in random directions, giving the object a matte look. Should the ray bounce and hit nothing, the pixel takes on the background colour at half intensity.
Because of float point imprecision, sometimes the intersection is calculated to be inside of the spheres, which immediately returns a black pixel. This is somwhat remedied by biasing the minimum collision distance to be a very small positive float instead of 0.0.

<img align="center" width="100%" src="./renderings/test_image_8-20.png" style="aspect-ratio: 16/9">

### Upping the number of ray bounces to be traced.
Upping the number of ray bounces and output gamma corrected RGB values.
The spheres appear grey here due to the lack of self colouring so they take on different intensities of black and the background colour.
<img align="center" width="100%" src="./renderings/test_image_8-25.png" style="aspect-ratio: 16/9">

### Higher sampling rate.
Less grainy texture due the the more stable averages of the RGB values.
<img align="center" width="100%" src="./renderings/test_image_8-29.png" style="aspect-ratio: 16/9">

### Add materials and ray scattering behaviour (lambertian and metals).
Left to right are: metal mirror, matte, semi-matte on metals.
<img align="center" width="100%" src="./renderings/test_image_9-5.png" style="aspect-ratio: 16/9">

### Add the dielectric class (glass material which can also refract).
<img align="center" width="100%" src="./renderings/test_image_10-4.png" style="aspect-ratio: 16/9">

### Add camera repositioning.
<img align="center" width="100%" src="./renderings/test_image_11-2.png" style="aspect-ratio: 16/9">

### Add depth of field.
<img align="center" width="100%" src="./renderings/test_image_12-2.png" style="aspect-ratio: 16/9">

### Randomly generated globes.
The big globe ate the smaller one 😩.
<img align="center" width="100%" src="./renderings/test_image_13.png" style="aspect-ratio: 16/9">

### Parallel Processed.
Fun with rayon/arc/send/sync 😑.
<img align="center" width="100%" src="./renderings/test_image_13_par.png" style="aspect-ratio: 16/9">

### Getting rid of collisions in world generation.
<img align="center" width="100%" src="./renderings/test_image_13_no_collision.png" style="aspect-ratio: 16/9">

