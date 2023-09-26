export function setNote(id: string | null) {
  location.hash = id ? `#/notes/${id}` : "#/";
}
