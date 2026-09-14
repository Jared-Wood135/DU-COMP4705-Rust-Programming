# COMP4705 - Rust Programming

This course was taken during University of Denver's Fall Quarter of 2026.

This REPO will house code from assignments/projects and notes created during the course by Jared Wood (Creator of this REPO).

**IT WILL NOT** house any lectures, instructions, or PII from the course and its respective instructor, teacher's assistant, students, and otherwise besides the creator of this REPO (Jared Wood).





## Table of Contents

- [Acknowledgements](#acknowledgements)
- [Environment Setup](#environment-setup)
- [Known Issues](#known-issues)





## Acknowledgements

[Back to Table of Contents](#table-of-contents)

- University of Denver's Fall Quarter 2026 (COMP4705 - Rust Programming)





## Environment Setup

[Back to Table of Contents](#table-of-contents)

**THIS IS NOT IMPLEMENTED AND IS A PLACEHOLDER FOR NOW!!!**

**TBD**

Python version in both environments: `VERSION HERE`

You have two options for setting up your Python environment:

### Option 1: Conda (Recommended)

**Conda** is an open-source environment and package manager that makes it easy to manage Python versions and dependencies. If you do not already use an environment manager, you may want to familiarize yourself with one since it helps avoid conflicts and makes reproducibility easier.  I use Conda and I think it's the easiest (Though I haven't used other packages)

**Steps:**
1. Install [Anaconda](https://www.anaconda.com/products/distribution) or [Miniconda](https://docs.conda.io/en/latest/miniconda.html).
2. Clone this repository (Or just download ```environment.yml```).
3. Create the environment using the provided `environment.yml`:
	```bash
	conda env create -f environment.yml
	conda activate 
	```

### Option 2: pip (Use with Caution)

You can also use `pip` with the `environment.txt` file. Using pip does not manage Python versions, so you must ensure your Python version matches the requirements.

**Steps:**
1. Ensure you are using a compatible Python version (see above).
2. Clone this repository (Or just download environment.txt).
3. Install dependencies:
	```bash
	pip install -r environment.txt
    ```





## Known Issues

[Back to Table of Contents](#table-of-contents)

- `environment.yml` and `environment.txt` is not created yet (TBD)
