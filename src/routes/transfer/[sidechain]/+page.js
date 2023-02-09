// src/routes/blog/[slug]/+page.js
export async function load({ params }) {
  const sidechain = params.sidechain;
  return {
    sidechain
  };
}
