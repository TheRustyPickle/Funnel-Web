# Pre-Release

## 01.2025

* Move to bar chart for User Chart
* Add option to add other users to the chart for Message and User chart
* Add a chart based on active user in a period on User Chart
* Add a chart based on message activity on Message Chart
* Add functionality to Common Words tab
* Add Channel Table tab with table for channels
* Reduce total iterations required to build the Overview chart
* Add deleted message and unique channel count on User Table tab
* Add handling for member joins and leave data on Overview tab
* Fix Start page uneven centering of buttons
* Add custom hover labels and x axis labels on Overview chart
* Add a chart on the Overview tab to visualize member count movement
* Add total member count handling on the Overview tab

## 12.2024

* Deploy the server to Shuttle, connect it with the UI with initial data to the database
* Update server side to handle more concurrent connections with higher efficiency by thread pooling
* Add animation to number changes in the Overview tab
* Add the UI on the Start page to show the change log
* Remove the password system on the initial UI and directly connect to the server using WebSocket
* Fix Vertical line vanishing from top tab UI when changing tabs
* Prevent reloading UI elements that are not selected currently and keep tab of pending reloads
* Added relevant method to send/receive member count data from server
* Update Overview tab to handle Comparison for total message and unique members
* Update egui to 30.0 and other dependencies to the latest version

## 11.2024

* Update Overview tab to handle messages and update UI elements

## 10.2024

* Removed JS codes for webworker and use Wasm native ehttp library
