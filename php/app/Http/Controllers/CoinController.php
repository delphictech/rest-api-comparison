<?php

namespace App\Http\Controllers;


use App\Models\Coin;
use Illuminate\Http\Request;


class CoinController extends Controller
{
    public function show(string $name)
    {
        $balance = Coin::getBalance($name);
        return ['data' => [
            'balance' => $balance
        ]
        ];
    }
}
