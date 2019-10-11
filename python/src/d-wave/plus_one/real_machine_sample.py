from dwave.system.samplers import DWaveSampler
from dwave.system.composites import EmbeddingComposite

sampler = EmbeddingComposite(DWaveSampler())
Q = {(0,0):-1, (0,1):2, (1,1):-1}
response = sampler.sample_qubo(Q, num_reads=10)
print(response)
