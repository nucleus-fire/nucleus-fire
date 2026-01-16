import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Next.js Bench",
  description: "Benchmark App",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
