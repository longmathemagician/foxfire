# Todo list
1) Context menu
   1) Copy option
   2) Set as background option
2) Fix mouse event issue when panning and mouse enters toolbar region
3) Refine button event code
   1) Add masking over painted areas
   2) Optimize paint calls
   3) Fix drag overlap bounds
4) Refine folder listing
   1) Only list images which can be opened
   2) Cache next and previous images for faster loading
   3) Soft disable / recolor next/previous buttons at ends of list
5) Review async code, remove blocking calls and replace with cached loading image
6) Rewrite draw code to use native surfaces & work on windows graphics cards which clamp rect math
7) Add fullscreen mode
8) Add readme