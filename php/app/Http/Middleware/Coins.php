<?php

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;
use App\Models\Coin;

class Coins
{

    public function handle(Request $request, Closure $next): Response
    {

        $authtoken = Coin::getAuthToken($request['name']);
        if ($request->header('authtoken') && $request->header('authtoken') == $authtoken) {
            return $next($request);
        } else {
            abort(403);
        }
    }
}
