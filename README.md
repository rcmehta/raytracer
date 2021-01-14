# raytracer
A raytracer built in rust, based on:

* [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
* [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)

with added parallelisation using rayon

# Final random image - Ray Tracing in One Weekend:

1200p, 3:2 resolution

500 rays with depth 50 for ray interactions

Render time ~30 min on Ryzen 5 1600

![final random image](https://github.com/rcmehta/raytracer/blob/main/render/final_random_image.png)

# Low sampled final mixed image - Ray Tracing The Next Week:

800p, 1:1 resolution

1000 rays with depth 50

![final mixed image](https://github.com/rcmehta/raytracer/blob/main/render/final_mixed_image.png)