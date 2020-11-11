defmodule RustlerSteiner.MixProject do
  use Mix.Project

  def project do
    [
      app: :rustler_steiner,
      version: "0.1.0",
      elixir: "~> 1.11",
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates(),
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.21.0"}
    ]
  end

  defp rustler_crates() do
    [
      steiner_tree: [
        path: "native/steiner_tree",
        mode: if(Mix.env() == :prod, do: :release, else: :debug)
      ]
    ]
  end
end
