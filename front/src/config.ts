export const apiUrl =
  location.search.match(/\?api=([^&]+)/)?.[1] ??
  import.meta.env.VITE_API_URL ??
  "http://localhost:8000/";
