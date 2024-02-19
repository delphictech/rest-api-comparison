<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class Coin extends Model
{
    protected static $data = [
        'alex' => [
            'authtoken' => 123,
            'balance' => 100
        ],
        'jason' => [
            'authtoken' => 234,
            'balance' => 200
        ],
        'marie' => [
            'authtoken' => 345,
            'balance' => 300
        ],
    ];

    public static function getBalance($name)
    {
        return isset(self::$data[$name]) ? self::$data[$name]['balance'] : null;
    }

    public static function getAuthToken($name)
    {
        return isset(self::$data[$name]) ? self::$data[$name]['authtoken'] : null;
    }
}
