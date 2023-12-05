## What is a Ray Tracer?

A ray-tracer is a program that generates images by simulating how light travels through a scene. It accomplishes this by calculating the paths of light rays and their interactions with various objects and materials within the scene. The process heavily relies on Linear Algebra for performing the necessary vector calculations.

### Process Overview:
1. **Ray Generation:** Rays originate from the camera. Why the camera? Starting at the light source would result in tracing numerous rays that never reach the camera lens, wasting computational resources. By originating rays from the camera (corresponding to each pixel), we focus on rays relevant to the final image.

2. **Ray-Object Interaction:** Once the rays are generated, calculations determine how they interact with objects in the scene. This interaction involves factors like hitting or being reflected by objects.

3. **Ray Reflection:** Reflected rays continue to be processed until they either exceed a defined reflection limit or interact with a light source.

4. **Light Calculation:** If a ray interacts with a light source, it contributes to the pixel's light calculation. This step considers all interactions the ray has encountered.

By repeating this process for every pixel in an image, ray-tracing produces stunning, realistic images by simulating the behavior of light in a scene.


## Technical Details

* **Language Used:** Rust
* **Custom Mathematical Types:** Developed custom mathematical types to achieve granular control and precision over various data types.
* **Implemented BRDFs (Bidirectional Reflectance Distribution Functions):** Supported multiple BRDFs to model different material properties and surface behaviors.
* **Recursive Core Algorithm:** Employed a recursive core algorithm for ray tracing, enabling the simulation of multiple interactions of light rays within the scene.
* **Anti-Aliasing:** Implemented anti-aliasing techniques to improve the visual quality of rendered images by reducing aliasing artifacts.
* **Linear Algebra Integration:** Utilized Linear Algebra extensively in the implementation, facilitating calculations and transformations necessary for simulating light interactions and object rendering.


![alt text](https://github.com/Jakersnell/SimpleRayTracing/blob/0648a442d7eba30647f32b33d43363476700cd37/output/mixed2.png)
Resources used.

    Essence of Linear Algebra, By 3blue1brown.
        https://youtu.be/fNk_zzaMoSs

    Scratchapixel.com
        https://www.scratchapixel.com/

    The Ray Tracer Challenge: Rendering soft shadows, By Jamis Buck.
        http://raytracerchallenge.com/bonus/area-light.html

    The Ray Tracer Challenge: Bounding Boxes and hierarchies, By Jamis Buck.
        http://www.raytracerchallenge.com/bonus/bounding-boxes.html

    Ray Tracing in One Weekend, By Peter Shirley.
        https://raytracing.github.io/
