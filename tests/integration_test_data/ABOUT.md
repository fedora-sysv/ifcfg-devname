## About integration testing of ifcfg_devname

Integration tests of ``ifcfg_devname`` uses multiple datasets to ensure correct behaviour of binary and correct results.


### List of datasets for integration testing

* [[``1``](./1/)] - Is ``ifcfg_devname`` able to get new device name from kernel cmdline - should [``PASS``]
* [[``2``](./2/)] - Is ``ifcfg_devname`` able to get new device name from ifcfg configuration - should [``PASS``]
* [[``3``](./3/)] - Missing configuration for new device name - should [``FAIL``]
* [[``4``](./4/)] - Missing configuration for new device name - should [``FAIL``]
