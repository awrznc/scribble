import dwave_networkx as dnx
import networkx as nx

import dimod # シュミレーション
# from dwave.system.samplers import DWaveSampler # 実機
# from dwave.system.composites import EmbeddingComposite # 実機

sampler = dimod.SimulatedAnnealingSampler() # シュミレーション
# sampler = EmbeddingComposite(DWaveSampler()) # 実機


G = nx.Graph()
G.add_edges_from([(0,1),(0,2),(1,3),(2,3),(3,4)])

# 頂点被覆問題をとくためのライブラリ（min_vertex_cover）
candidate = dnx.min_vertex_cover(G, sampler)
print(candidate)
