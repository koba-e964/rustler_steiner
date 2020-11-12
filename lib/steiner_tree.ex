defmodule SteinerTree do
  use Rustler, otp_app: :rustler_steiner, crate: :steiner_tree

  @type edges() :: [{integer(), integer()}]

  # Computes a steiner tree for a given graph.
  @spec compute(integer(), edges()) ::
          {:ok, edges()}
          | {:error,
             reason :: {:too_large_input, integer()} | {:invalid_arg, {integer(), edges()}}}
  def compute(_n, _edges), do: exit(:nif_not_loaded)
end
