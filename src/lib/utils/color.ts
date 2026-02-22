export async function getDominantColor(
  imageUrl: string,
): Promise<string | null> {
  return new Promise((resolve) => {
    const img = new Image();
    img.crossOrigin = "Anonymous";
    img.src = imageUrl;

    img.onload = () => {
      const canvas = document.createElement("canvas");
      const ctx = canvas.getContext("2d");

      if (!ctx) {
        resolve(null);
        return;
      }

      // average colors using this
      canvas.width = 1;
      canvas.height = 1;
      ctx.drawImage(img, 0, 0, 1, 1);

      try {
        const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
        resolve(`rgb(${r}, ${g}, ${b})`);
      } catch (e) {
        console.error("Failed to extract color:", e);
        resolve(null);
      }
    };

    img.onerror = () => {
      resolve(null);
    };
  });
}
