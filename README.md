# jiggle

let's get the show on the road:

- a bunch of sorta functions:
    - wasm modules, must define `run` function:
        - that takes arbitrary bin data as input,
        - and optionally produces arbitrary bin data as output.
    - function is:
        - run in a separate thread
        - cap put in place on execution time
