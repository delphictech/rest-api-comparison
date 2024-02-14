<?php

use App\Http\Controllers\CoinController;
use App\Http\Middleware\Coins;
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
    return 'HELLO WORLD';
});

Route::get('/test', function () {
    return [
        'message' => 'testing route',
        'code' => 200
    ];
});

Route::get('/coins/{name}', [CoinController::class, 'show'])
    ->middleware(Coins::class);
