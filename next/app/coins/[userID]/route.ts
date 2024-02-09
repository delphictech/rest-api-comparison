import { headers } from "next/headers";
import { NextRequest, NextResponse } from "next/server";

const mockLoginDetails: Record<string, { authToken: string, userName: string }> = {
    alex: {
      authToken: "123",
      userName: "alex",
    },
    jason: {
      authToken: "234",
      userName: "jason",
    },
    marie: {
      authToken: "345",
      userName: "marie",
    },
};
const mockCoinDetails: Record<string, { balance: number }> = {
    alex: {
        balance: 100,
    },
    jason: {
        balance: 200,
    },
    marie: {
        balance: 300,
    },
};

export async function GET(_: NextRequest, { params }: { params: { userID: string } }) {
    // get authorization
    const headersList = headers();
    const authToken = headersList.get("authtoken");
    const authDetails = mockLoginDetails[params.userID];
    
    if (authDetails.authToken === authToken) return NextResponse.json(mockCoinDetails[authDetails.userName]);
    return NextResponse.error();
}