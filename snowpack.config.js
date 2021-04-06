module.exports = {
  workspaceRoot: false,
  mount: {
    www: '/',
  },
  optimize: {
    minify: true,
    target: 'esnext',
  },
  plugins: ['@snowpack/plugin-typescript'],
};
