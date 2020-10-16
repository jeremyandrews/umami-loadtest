# Overview

This is a load test for Drupal's Umami Profile, which is included as part of Drupal 9 core. In order to use this load test, you must first install the Umami Profile as documented here:
https://www.drupal.org/docs/umami-drupal-demonstration-installation-profile

The load test was developed using a locally hosted Drupal 9 install hosted in a DDEV container:
https://www.ddev.com/

## Load Test Implementation

The load test is split into the following files:
 - `main.rs`: This file contains the main() function and defines the actual load test.
 - `common.rs`: This file contains helper functions used by the task functions.
 - `english.rs`: This files contains all task functions loading pages in English.
 - `spanish.rs`: This files contains all task functions loading pages in Spanish.