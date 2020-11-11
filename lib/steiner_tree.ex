defmodule SteinerTree do
  use Rustler, otp_app: :rustler_steiner, crate: :steiner_tree

  # Computes a steiner tree for a given graph.
  def compute(_n, _edges), do: exit(:nif_not_loaded)
end
