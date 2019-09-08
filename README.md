# Sorting Visualiser

### Visualisations:

There are two types of visualisations; ones that use a single array, and ones that use multiple arrays. I will mark ones that use multiple arrays with an (M).
You can switch between visualisations while sorting IF you are switching to a visualisation of the same type.
More visualisations are to come in the future. :)

#### Circle:

Displays array in a circle:

<img src="https://github.com/eggmund/sort_visualiser/blob/master/readme_images/circle_vis.png" alt="Circular Visualisation" width="475" height="474">

#### Bars:

Displays array as a row of bars:

<img src="https://github.com/eggmund/sort_visualiser/blob/master/readme_images/bar_vis.png" alt="Bar Visualisation" width="475" height="474">

Coloring:

**Element** | **Colour**
--- | ---
Active | Blue
Secondary Active (used when comparing elements) | Blue
Pivot (Quicksort) | Purple

Colours may change in the future.

With Lomuto partitioning quicksort (default quicksort implemented), the two active elements show the area where the elements are collecting that are bigger than the pivot. Once it reaches the end of the partition it moves the pivot to before that area.

#### Dots:

Displays array as dots (looks good with quick sort):

<img src="https://github.com/eggmund/sort_visualiser/blob/master/readme_images/dot_vis.png" alt="Dot Visualisation" width="475" height="474">

Exact same colouring as the bar visualisation.

#### Pixels (M):

Displays multiple arrays, spanning from the left to the right of the window. Each row of pixels is a seperate array.

<img src="https://github.com/eggmund/sort_visualiser/blob/master/readme_images/pixel_vis.png" alt="Pixel Visualisation" width="475" height="474">

<img src="https://github.com/eggmund/sort_visualiser/blob/master/readme_images/pixel_vis_shuffled.png" alt="Pixel Visualisation Shuffled" width="475" height="474">

Does not display active elements etc because it would be a bit too cluttered.

### Controls:
#### Sorts:
**Key** | **Sort**
--- | ---
**1** | Bubble Sort.
**2** | Insertion Sort.
**3** | Cocktail Shaker Sort.
**4** | Quicksort (Lomuto partitioning).
**5** | Merge Sort.
**6** | Shell Sort.
**7** | Comb Sort (very similar to shell sort).
**8** | Radix LSD Sort (Base 10).

You can do multiple sorts at once but be careful since this can ruin the array (however you can reset by pressing **R**).

NOTE: Due to Quicksort's Lomuto partitioning scheme, sorting the sorted or reversed array is incredibly slow, and is a key problem with this partitioning scheme, since it uses the last element as the pivot, rather than the middle.

#### Array functions:
**Key** | **Sort**
--- | ---
**S** | Shuffle.
**R** | Reset array.
**I** | Invert/reverse array.
**Q** | Cancel current sort.

Resetting the array regenerates all of the elements in the array, so if you ever have any problems with the array, for example duplicate array elements due to running multiple sorts, then reset the array and you should be good to go.

#### Display modes:
**Key** | **Sort**
--- | ---
**C** | Circle.
**B** | Bars.
**D** | Dots.
**P** | Pixels.

### Configuration:

Radix sort bases and sleep time for each sort can be changed, 

### Compiling and Running:

Install rust (rustup + cargo), then change directory to this folder, and run:

```bash
cargo build --release
```

Then you will find the executable in `target/release/sort_visualiser`.

If you have a problem with linking, so `shaderc-sys` cannot compile, then build it with the `--features=shaderc_fix` flag.