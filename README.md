# wprofiler
A console based executable to get some more info on Wwise performance monitor data

## getting the data
First you have to get the performance graph data form Wwise.
You do this right clicking inside of the Performance Monitor (in the profiler layout) after you profiled the project,
Then you click the `Profiler/Save Performance Counters` button and choose a location to save it.


## usage
To use the exe you must either add the directory to the environment path and call `wprofiler.exe` from your terminal,
Or move the desired "Performance Monitor.txt" into the same directory as the executable and call `./wprofiler.exe`from there.

It takes one argument which is the file name.
- example: `./wprofiler.exe "Performance Monitor.txt"`

It will then (hopefully) create a new file called "formatted.json" which contains the min, max and average of each column.
Errors will be shown in the console too.

It works for Wwise 2021.1.13, haven't tested any other versions.