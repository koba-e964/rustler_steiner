defmodule SteinerTree do
  use Rustler, otp_app: :rustler_steiner, crate: :steiner_tree

  @type edges() :: [{integer(), integer()}]

  # Computes a steiner tree for a given graph.
  @spec compute(integer(), edges(), [integer()]) ::
          {:ok, {integer(), edges()}}
          | {:error,
             reason :: {:too_large_input, integer()} | {:invalid_arg, {integer(), edges()}}}
  def compute(_n, _edges, _terms), do: exit(:nif_not_loaded)
  # Computes a steiner tree for a given graph. This function is non-yielding.
  @spec compute_nonyielding(integer(), edges(), [integer()]) ::
          {:ok, {integer(), edges()}}
          | {:error,
             reason :: {:too_large_input, integer()} | {:invalid_arg, {integer(), edges()}}}
  def compute_nonyielding(_n, _edges, _terms), do: exit(:nif_not_loaded)
end
