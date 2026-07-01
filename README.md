<p align="center">
  <picture>
      <source media="(prefers-color-scheme: dark)" srcset="./docs/aurorite-dark.svg">
      <source media="(prefers-color-scheme: light)" srcset="./docs/aurorite-light.svg">
      <img alt="aurorite logo" src="./docs/aurorite-dark.svg">
    </picture>
    <br>
    <img src="https://github.com/justxale/aurorite/actions/workflows/cargo-test.yml/badge.svg" alt="tests"/>
</p>

---

Aurorite is the self-hosted solution for your DnD campaigns. Create your characters with builder and play with your
friends using built-in DnD 5e package! Or create your own edition using customizable node-based programming!

### Installation
The easiest way to install Aurorite is using Docker:
```shell
git clone https://github.com/justxale/aurorite.git
cd aurorite
docker build -t aurorite:latest .
docker run -e AURORITE_SECRET=totallysecret! aurorite:latest
```