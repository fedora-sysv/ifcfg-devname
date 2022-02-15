# About integration testing of ifcfg-devname

Integration tests of ``ifcfg-devname`` uses multiple datasets to ensure correct behavior of binary and correct results.

## List of datasets for integration testing

* [[``1``](./1/)] - Missing ifcfg configuration for new device name - should [``FAIL``]
* [[``2``](./2/)] - Is ``ifcfg-devname`` able to get new device name from ifcfg configuration - should [``PASS``]
* [[``3``](./3/)] - Missing ifcfg configuration files - should [``FAIL``]
* [[``6``](./6/)] - Whitespaces in ifcfg files - should [``PASS``]
* [[``7``](./7/)] - Comments (``#``) in ifcfg files - should [``FAIL``]
