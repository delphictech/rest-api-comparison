# Learning PHP REST API Project

This is a basic PHP project created for learning purposes. The project focuses on making REST API calls and implementing middleware for a simple web application.

## Getting Started

### Prerequisites

Ensure you have PHP and composer and  installed on your machine. Furthermore it is also recommended to have NPM and Node.js installed.

If not, follow the instructions for installing PHP regading the [PHP website](https://www.php.net/manual/en/install.php).
Since the documentation on the official website can be a little bit overwhelming, especially for Windows,
feel free to watch a [youtube video](https://www.youtube.com/watch?v=MPRLUd8Pmyo) which guides you through the PHP installation.

Check if PHP is installed in your terminal before proceeding:

```bash
php -v
```

After that, follow the instructions for installing composer regarding the [Composer  website](https://getcomposer.org/doc/00-intro.md).
If you feel overwhelmed with this documentation, feel free to look a youtube video and do the steps provided there.

Check if composer is installed in your terminal before proceeding:

```bash
composer -v
```
For instructions regarding installing Node.js and NPM, follow the instructions listed in the Node folder of this repo.

### Running Project

```bash
php artisan serve
```

This will run the server at localhost:8000.

## Resources

### Language Project layout resources

This example covers PHP with the Framework Laravel. For most backend related work it is highly recommended to use a framework.
Laravel is by far the most used PHP framework out there and getting started with Laravel is easy if you have common knowledge in PHP.
When installing a new Laravel project, you already get a fully structured project. If you are curious what all the folders are, feel free to read the following documentation.
  - [Laravel Framework Project Structure](https://laravel.com/docs/10.x/structure)

Regarding this specific project, there are a lot of folders and files not necessarily needed, but since they come out of the box with a fresh laravel installment,
we decided to leave them here. The files that were modified to build this demo and that you should look for are listed below.
  - app/Http/Controllers/CoinController.php
  - app/Http/Middleware/Coins.php
  - app/Models/Coin.php
  - routes/web.php

### Language Specifics

It is recommended to use a framework to speed up development time and make use of already existing functions.
Remember, programmers are lazy and if you can use things like a framework to speed things up, you should do.
That being said, it is beneficial to first gain a basic knowledge of the language itself before using a framework.
Here is a good ressource that teaches you most of the things you need to know:
- [PHP for Beginners Full Course](https://www.youtube.com/watch?v=fw5ObX8P6as&t=0s)
After you watched this course, you should be ready to start learning Laravel.


#### Summary

PHP is by far the most used back end language out there. This is due to PHP being one of the first backend languages and many old websites being build with PHP.
Also, Content Management Systems like Wordpress, which give people the possibility to create websites without knowing how to code, are build upon PHP.
This being said, there are also cons in using PHP as a backend language, e.g. performance.
To give you a better picture, here are a few links:
  - [PHP vs JavaScript](https://medium.com/geekculture/javascript-vs-php-the-best-language-for-backend-development-79d41736a279#:~:text=PHP%20is%20the%20abbreviation%20for,pages%20and%20database%2Ddriven%20sites)
  - [PHP vs Golang](https://www.tutorialspoint.com/difference-between-golang-and-php)
  - [PHP vs Python](https://kinsta.com/blog/php-vs-python/)
