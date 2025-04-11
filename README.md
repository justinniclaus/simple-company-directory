# Company Directory

A simple command-line application built in Rust that helps organize employees by department.

## Description

This Rust application allows users to create a company directory by entering employee names and their corresponding departments. The program maintains an organized list and displays the directory after each entry, with departments sorted alphabetically.

## Features

- Add employees to departments interactively
- Automatically organizes employees by department
- Displays an alphabetically sorted directory after each entry
- Simple command-line interface

## Installation

### Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust installation)

## Usage

1. When prompted, enter an employee name followed by their department.
   Example: `John Engineering`

2. Continue adding employees as needed.

3. The directory will be displayed after each entry, showing all departments and their employees.

4. Type `exit` to quit the program.

Example session:
```
Enter name and department:
Type exit to stop.
John Engineering
----Company Directory----
Engineering: John
Mary Marketing
----Company Directory----
Engineering: John
Marketing: Mary
exit
```

## How It Works

The application uses a HashMap to store departments as keys and a vector of employee names as values. This structure allows for efficient organization and retrieval of employees by department.

Key components:
- HashMap for storing department-employee relationships
- Vector for maintaining lists of employees in each department
- Sorting functionality to display departments alphabetically
