import { mockLoginDetails } from "@/app/coins/[userID]/route";
import { NextRequest, NextResponse } from "next/server";

export function middleware(request: NextRequest) {
    if (request.nextUrl.pathname.startsWith('/coins')) {
        // get authorization
        const headersList = request.headers;
        const authToken = headersList.get("authtoken");
        const requestedUser = request.nextUrl.pathname.split("/").at(-1) || "none";
        const userAuthToken = mockLoginDetails[requestedUser].authToken;
        if (authToken !== userAuthToken) return NextResponse.json({ error: "Unauthorized User" }, { status: 403 });;
    }
};

export const config = {
    matcher: [
      /*
       * Match all request paths except for the ones starting with:
       * - _next/static (static files)
       * - _next/image (image optimization files)
       * - favicon.ico (favicon file)
       */
      {
        source: '/((?!_next/static|_next/image|favicon.ico).*)',
        missing: [
          { type: 'header', key: 'next-router-prefetch' },
          { type: 'header', key: 'purpose', value: 'prefetch' },
        ],
      },
    ],
}