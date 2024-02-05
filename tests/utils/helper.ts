import axios from "axios";

/**
 * used for fetching API data or throws an error
 *
 * @export
 * @param {string} URL - API route
 * @param {Record<string, string>} [headers] - optional headers input to axios (needed for setting auth token)
 * @return {*}
 */
export async function fetchDataFromAPI(
  URL: string,
  headers?: Record<string, string>
) {
  try {
    const response = await axios.get(URL, { headers });

    return response.data;
  } catch (error) {
    console.log("ERROR", error);
    throw error;
  }
}
