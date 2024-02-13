<?php

use App\Http\Middleware\Token;
use Illuminate\Support\Facades\Route;

/*
|--------------------------------------------------------------------------
| Web Routes
|--------------------------------------------------------------------------
|
| Here is where you can register web routes for your application. These
| routes are loaded by the RouteServiceProvider and all of them will
| be assigned to the "web" middleware group. Make something great!
|
*/

Route::get('/', function () {
    return 'Hello from Laravel!';
});

Route::get('/test', function () {
    return [
        'message' => 'Hello again!',
        'code' => 200
    ];
});

Route::get('/coins', function() {
    return 'middlo';
})->middleware(Token::class);
