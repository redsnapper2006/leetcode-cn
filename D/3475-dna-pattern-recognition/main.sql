select
  sample_id,
  dna_sequence,
  species,
  if (INSTR (dna_sequence, 'ATG') = 1, 1, 0) as has_start,
  if (
    if (RIGHT (dna_sequence, 3) = 'TAA', 1, 0) + if (RIGHT (dna_sequence, 3) = 'TAG', 1, 0) + if (RIGHT (dna_sequence, 3) = 'TGA', 1, 0) = 1,
    1,
    0
  ) as has_stop,
  if (INSTR (dna_sequence, 'ATAT'), 1, 0) as has_atat,
  if (INSTR (dna_sequence, 'GGG'), 1, 0) as has_ggg
from
  Samples
order by
  sample_id
