export const SERVER = !process.env.NODE_ENV || process.env.NODE_ENV === 'development'
    ? "http://localhost:8080"
    : window.location.origin;

export async function cleanUrl(url) {
    var result = await fetch(`${SERVER}?${url}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        }
    });
    return await result.json();
}