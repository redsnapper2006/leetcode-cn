package main

import "fmt"

func gridIllumination(N int, lamps [][]int, queries [][]int) []int {
	lampsBuf := make(map[int]int)

	lightsRow := make(map[int]int)
	lightsCol := make(map[int]int)
	lightsCross := make(map[int]int)
	lightsReverseCross := make(map[int]int)
	for i := 0; i < len(lamps); i++ {
		r, c := lamps[i][0], lamps[i][1]
		lampsBuf[r*N+c] = 1
	}
	for i := 0; i < len(lamps); i++ {
		row, col := lamps[i][0], lamps[i][1]
		lightsRow[row]++
		lightsCol[col]++
		lightsCross[row-col+N-1]++
		lightsReverseCross[row+col]++
	}

	cord := [][]int{[]int{-1, -1}, []int{-1, 0}, []int{-1, 1}, []int{0, -1}, []int{0, 0}, []int{0, 1}, []int{1, -1}, []int{1, 0}, []int{1, 1}}
	var ret []int
	for i := 0; i < len(queries); i++ {
		row, col := queries[i][0], queries[i][1]
		state := 0

		if lightsRow[row] > 0 || lightsCol[col] > 0 || lightsCross[row-col+N-1] > 0 || lightsReverseCross[row+col] > 0 {
			state = 1
		}
		ret = append(ret, state)
		for cordIdx := 0; cordIdx < len(cord); cordIdx++ {
			neiRow, neiCol := row+cord[cordIdx][0], col+cord[cordIdx][1]
			if neiRow >= 0 && neiRow < N && neiCol >= 0 && neiCol < N && lampsBuf[neiRow*N+neiCol] == 1 {
				lampsBuf[neiRow*N+neiCol] = 0

				lightsRow[neiRow]--
				lightsCol[neiCol]--
				lightsCross[neiRow-neiCol+N-1]--
				lightsReverseCross[neiRow+neiCol]--
			}
		}
	}
	return ret
}

func main() {
	fmt.Println(gridIllumination(10000,
		[][]int{[]int{6781, 8653}, []int{2624, 6531}, []int{7485, 1378}, []int{3819, 4964}, []int{7741, 343}, []int{9960, 5}, []int{83, 1173},
			[]int{9533, 9441}, []int{1763, 8712}, []int{2082, 6100}, []int{4538, 9471}, []int{9093, 9315}, []int{3415, 1120}, []int{8646, 7698},
			[]int{7660, 4705}, []int{6683, 9643}, []int{3665, 9248}, []int{685, 6302}, []int{8082, 4077}, []int{9369, 91}, []int{9404, 5771},
			[]int{7233, 9561}, []int{3236, 8674}, []int{6280, 3683}, []int{4672, 272}, []int{9098, 7894}, []int{9051, 7955}, []int{6721, 5461},
			[]int{1525, 4768}, []int{2305, 3189}, []int{1425, 1664}, []int{7277, 5281}, []int{5270, 94}, []int{9135, 1047}, []int{1423, 2026},
			[]int{9364, 2504}, []int{1387, 9993}, []int{4382, 9260}, []int{1981, 9858}, []int{7104, 6716}, []int{4844, 7154}, []int{1974, 1734},
			[]int{5252, 1305}, []int{122, 6845}, []int{6148, 6059}, []int{7072, 4871}, []int{5503, 9185}, []int{2443, 8877}, []int{6763, 4757},
			[]int{1733, 270}, []int{7044, 953}, []int{2843, 7884}, []int{3418, 7137}, []int{3455, 1437}, []int{8334, 1716}, []int{1941, 3179},
			[]int{5258, 2367}, []int{3446, 3109}, []int{1750, 4210}, []int{6160, 7724}, []int{995, 3633}, []int{2915, 5399}, []int{6484, 6788},
			[]int{2869, 1759}, []int{7072, 4567}, []int{7647, 5011}, []int{6274, 9931}, []int{2319, 6246}, []int{1785, 7459}, []int{2916, 7772},
			[]int{9800, 1224}, []int{1349, 3521}, []int{6343, 5734}, []int{7690, 3297}, []int{4464, 4156}, []int{4481, 4577}, []int{5123, 8000},
			[]int{9307, 5493}, []int{434, 3497}, []int{156, 2584}, []int{3350, 9427}, []int{9518, 8519}, []int{6584, 9086}, []int{8670, 3110},
			[]int{5609, 704}, []int{8206, 9545}, []int{234, 2094}, []int{2546, 5708}, []int{1748, 775}, []int{6867, 3292}, []int{418, 8985},
			[]int{2143, 8460}, []int{9026, 4618}, []int{6422, 8113}, []int{776, 7424}, []int{4863, 246}, []int{9547, 9529}, []int{7281, 3801},
			[]int{1158, 756}, []int{3594, 6972}, []int{2104, 8230}, []int{6284, 9381}, []int{8005, 663}, []int{4891, 7678}, []int{5986, 6802},
			[]int{7632, 5781}, []int{252, 1032}, []int{6553, 1117}, []int{6802, 2078}, []int{7254, 6108}, []int{8183, 6906}, []int{4270, 779},
			[]int{7025, 5290}, []int{8568, 3721}, []int{5037, 5117}, []int{3762, 936}, []int{5608, 7927}, []int{1806, 239}, []int{311, 7986},
			[]int{2336, 2070}, []int{5936, 3448}, []int{2625, 3869}, []int{3151, 3786}, []int{8015, 4709}, []int{9602, 3628}, []int{9979, 8901},
			[]int{1230, 4326}, []int{7385, 1126}, []int{4807, 8996}, []int{8851, 3992}, []int{8683, 4647}, []int{2647, 8763}, []int{8057, 7091},
			[]int{6324, 6119}, []int{2194, 6463}, []int{6001, 116}, []int{7471, 3912}, []int{695, 5827}, []int{1466, 2573}, []int{8290, 8915},
			[]int{6643, 964}, []int{6323, 2508}, []int{6348, 8692}, []int{65, 1256}, []int{3502, 9145}, []int{7699, 8757}, []int{7314, 1844},
			[]int{3271, 2660}, []int{1193, 632}, []int{5871, 6397}, []int{4997, 1054}, []int{2783, 2249}, []int{6893, 6290}, []int{7360, 102},
			[]int{1517, 627}, []int{3728, 1143}, []int{8739, 6919}, []int{9280, 5474}, []int{4628, 2357}, []int{228, 7040}, []int{501, 5529},
			[]int{2103, 816}, []int{1410, 3635}, []int{2532, 4676}, []int{9930, 1336}, []int{4702, 1074}, []int{1792, 4768}, []int{8121, 1924},
			[]int{332, 294}, []int{2014, 3097}, []int{6920, 8347}, []int{9979, 1698}, []int{1124, 1460}, []int{6289, 1525}, []int{2915, 7575},
			[]int{5605, 2174}, []int{8012, 6466}, []int{1183, 921}, []int{7881, 5492}, []int{3485, 3321}, []int{315, 5026}, []int{6193, 292},
			[]int{5492, 1956}, []int{2111, 7093}, []int{9995, 9861}, []int{9793, 7785}, []int{12, 9430}, []int{4259, 7394}, []int{3270, 8235},
			[]int{9558, 1799}, []int{3531, 460}, []int{6737, 322}, []int{2659, 7894}, []int{4699, 9552}, []int{1449, 3896}, []int{8499, 5173},
			[]int{396, 6550}, []int{1769, 198}, []int{6528, 4682}, []int{6130, 460}, []int{5227, 462}, []int{1143, 1800}, []int{3752, 8579},
			[]int{6214, 7897}, []int{9623, 4844}, []int{6368, 8903}, []int{5291, 6804}, []int{4505, 9788}, []int{458, 260}, []int{3388, 4934},
			[]int{6833, 4846}, []int{2436, 87}, []int{1086, 1797}, []int{3092, 7125}, []int{2628, 9746}, []int{5904, 2234}, []int{4650, 5985},
			[]int{5250, 9780}, []int{9164, 5137}, []int{6726, 4422}, []int{5561, 520}, []int{4030, 3670}, []int{6003, 3517}, []int{9966, 8589},
			[]int{6498, 636}, []int{97, 554}, []int{7217, 2598}, []int{9522, 6511}, []int{5071, 9806}, []int{7012, 3394}, []int{2384, 4573},
			[]int{3595, 3294}, []int{5002, 9070}, []int{2241, 5584}, []int{7704, 7891}, []int{6674, 7178}, []int{5770, 1054}, []int{4864, 4851},
			[]int{1795, 3449}, []int{6943, 116}, []int{8634, 639}, []int{9930, 8509}, []int{154, 7040}, []int{5772, 1086}, []int{7602, 9185},
			[]int{3224, 6686}, []int{3749, 9433}, []int{6626, 1384}, []int{490, 9302}, []int{5188, 5225}, []int{6477, 6470}, []int{7722, 6323},
			[]int{7165, 8446}, []int{9894, 5902}, []int{3324, 3383}, []int{4493, 8417}, []int{3042, 3611}, []int{8999, 6029}, []int{9498, 1661},
			[]int{8915, 1603}, []int{7364, 8270}, []int{2657, 2536}, []int{5777, 3673}, []int{3078, 8}, []int{9146, 5080}, []int{5627, 7425},
			[]int{2298, 4460}, []int{5071, 8675}, []int{1503, 9017}, []int{9393, 9495}, []int{445, 1479}, []int{7958, 837}, []int{2699, 3826},
			[]int{3961, 5356}, []int{3779, 7915}, []int{1413, 846}, []int{6134, 5782}, []int{5597, 1889}, []int{9750, 6650}, []int{2470, 8911},
			[]int{1835, 6724}, []int{4823, 9159}, []int{6015, 68}, []int{3007, 4122}, []int{5426, 5949}, []int{1791, 2750}, []int{6726, 2505},
			[]int{3916, 6447}, []int{3971, 9812}, []int{863, 1833}, []int{245, 2972}, []int{1684, 7871}, []int{7673, 8904}, []int{318, 5543},
			[]int{6260, 7149}, []int{2162, 3020}, []int{2588, 1706}, []int{3255, 2907}, []int{988, 2270}, []int{9372, 3622}, []int{4213, 2732},
			[]int{9602, 4788}, []int{4200, 2110}, []int{1310, 4719}, []int{5964, 7535}, []int{8105, 2500}, []int{5198, 8880}, []int{2048, 6247},
			[]int{9740, 7652}, []int{8931, 3532}, []int{7897, 8163}, []int{7546, 1375}, []int{6605, 2779}, []int{8731, 1869}, []int{5570, 458},
			[]int{3478, 2169}, []int{4984, 4294}, []int{778, 7288}, []int{3073, 1717}, []int{7404, 3605}, []int{3526, 3141}, []int{5623, 7086},
			[]int{2016, 6469}, []int{8791, 788}, []int{6063, 42}, []int{1501, 6271}, []int{2327, 4747}, []int{2158, 6952}, []int{8366, 6934},
			[]int{6196, 8766}, []int{2048, 537}, []int{9419, 8882}, []int{2699, 1465}, []int{6069, 90}, []int{6531, 2846}, []int{1559, 1665},
			[]int{2879, 5109}, []int{8458, 2757}, []int{9002, 7949}, []int{2091, 888}, []int{3379, 731}, []int{9819, 9121}, []int{4253, 758},
			[]int{9807, 5656}, []int{1312, 3355}, []int{5105, 7725}, []int{8134, 8266}, []int{541, 5857}, []int{3423, 2439}, []int{3573, 2907},
			[]int{7965, 9658}, []int{1518, 4452}, []int{8489, 559}, []int{1425, 27}, []int{4476, 4551}, []int{1130, 2435}, []int{7415, 8389},
			[]int{5212, 2528}, []int{6667, 8146}, []int{7084, 5134}, []int{7378, 6186}, []int{8498, 3450}, []int{20, 5565}, []int{5515, 758},
			[]int{5667, 2847}, []int{3977, 6921}, []int{6928, 6328}, []int{9137, 1581}, []int{4963, 4132}, []int{5452, 7456}, []int{3767, 6595},
			[]int{1527, 803}, []int{1483, 3482}, []int{915, 46}, []int{1794, 5649}, []int{8517, 3034}, []int{4308, 9060}, []int{3527, 1669},
			[]int{6091, 2994}, []int{9653, 2482}, []int{2087, 4577}, []int{7489, 3750}, []int{7354, 5719}, []int{4320, 332}, []int{7101, 3522},
			[]int{5531, 8162}, []int{5449, 9074}, []int{2831, 7322}, []int{5615, 8154}, []int{1821, 6090}, []int{6865, 3666}, []int{3194, 9916},
			[]int{1878, 7672}, []int{3456, 1615}, []int{7354, 8810}, []int{9820, 8548}, []int{9291, 2685}, []int{1166, 9665}, []int{5229, 5466},
			[]int{9324, 898}, []int{7015, 6267}, []int{3647, 9987}, []int{4612, 3779}, []int{5444, 5015}, []int{7812, 2419}, []int{5580, 6212},
			[]int{2024, 4262}, []int{7708, 4629}, []int{7529, 6246}, []int{9742, 3719}, []int{1345, 3956}, []int{2574, 572}, []int{7296, 5764},
			[]int{7535, 7323}, []int{4569, 4718}, []int{6804, 8445}, []int{9074, 5503}, []int{5030, 2626}, []int{5800, 4022}, []int{7268, 2570},
			[]int{8520, 87}, []int{8239, 1933}, []int{4823, 5979}, []int{2282, 3636}, []int{6428, 334}, []int{5712, 1325}, []int{9392, 8608},
			[]int{3197, 2394}, []int{69, 6421}, []int{2116, 1683}, []int{1628, 1621}, []int{8392, 2191}, []int{4557, 2476}, []int{6947, 7855},
			[]int{8483, 5211}, []int{7858, 8404}, []int{8429, 3125}, []int{2278, 3780}, []int{4415, 1443}, []int{8132, 4409}, []int{1631, 8065},
			[]int{3446, 1025}, []int{5720, 6030}, []int{4041, 3455}, []int{6554, 8496}, []int{1053, 2889}, []int{2056, 3502}, []int{104, 9720},
			[]int{4216, 4404}, []int{1923, 1595}, []int{3607, 450}, []int{107, 1444}, []int{8131, 9533}, []int{3953, 5001}, []int{7999, 1197},
			[]int{6747, 9860}, []int{4498, 9086}, []int{769, 6084}, []int{184, 2831}, []int{9984, 6442}, []int{735, 8677}, []int{9626, 4269},
			[]int{3132, 9705}, []int{3541, 2726}, []int{9620, 694}, []int{4040, 9332}, []int{339, 4012}, []int{8689, 9722}, []int{4392, 7520},
			[]int{849, 3020}, []int{215, 9738}, []int{701, 6102}, []int{4230, 229}, []int{2927, 8437}, []int{5152, 578}, []int{2100, 1291},
			[]int{9621, 4853}, []int{2124, 2991}, []int{3960, 5623}, []int{5837, 59}, []int{2559, 1426}, []int{9248, 9085}, []int{9031, 3224},
			[]int{1794, 9062}, []int{2446, 7485}, []int{4981, 5662}, []int{7495, 9944}, []int{4780, 5726}, []int{6185, 7062}, []int{1138, 1380},
			[]int{8560, 2631}, []int{5092, 4931}, []int{6891, 3631}, []int{4882, 8416}, []int{5612, 8966}, []int{658, 694}, []int{219, 3353},
			[]int{9021, 787}, []int{3172, 2423}, []int{6257, 3369}, []int{2980, 9236}, []int{5677, 9566}, []int{783, 8937}, []int{2776, 9980},
			[]int{1917, 850}, []int{4058, 833}, []int{617, 2159}, []int{1552, 8832}, []int{273, 3203}, []int{3517, 1183}, []int{5305, 2792},
			[]int{5092, 7799}, []int{2760, 7785}, []int{8591, 8919}, []int{5384, 6103}, []int{4507, 6209}, []int{874, 133}, []int{41, 396},
			[]int{624, 2028}, []int{991, 9392}, []int{2658, 2472}, []int{8837, 5917}, []int{3802, 1064}, []int{9337, 3044}, []int{5383, 7682},
			[]int{219, 7436}, []int{7022, 3740}, []int{3841, 4319}, []int{9924, 2068}, []int{9300, 5354}, []int{6357, 6890}, []int{3138, 6720},
			[]int{8337, 4385}, []int{634, 2952}, []int{6281, 9985}, []int{8483, 3579}, []int{8774, 3368}, []int{5765, 3143}, []int{7331, 4164},
			[]int{7662, 4750}, []int{3054, 7082}, []int{1470, 9351}, []int{6843, 387}, []int{5099, 7932}, []int{5169, 4010}, []int{6638, 5691},
			[]int{6299, 7519}, []int{9381, 38}, []int{183, 183}, []int{2223, 4947}, []int{7803, 6025}, []int{6851, 876}, []int{5067, 3842},
			[]int{9241, 1103}, []int{9412, 3986}, []int{5055, 2351}, []int{9301, 2109}, []int{405, 6305}, []int{3216, 3036}, []int{5329, 9696},
			[]int{6926, 8025}, []int{4428, 9866}, []int{952, 1974}, []int{2724, 9889}, []int{959, 7191}, []int{3992, 3275}, []int{2244, 6675},
			[]int{815, 6035}, []int{16, 6919}, []int{4401, 3365}, []int{8926, 6899}, []int{5276, 4517}, []int{3404, 3003}, []int{5648, 1545},
			[]int{8846, 5655}, []int{9028, 379}, []int{5371, 707}, []int{1167, 3576}, []int{5710, 7696}, []int{8879, 451}, []int{6646, 337},
			[]int{1350, 5554}, []int{5382, 6383}, []int{8887, 4528}, []int{1846, 4966}, []int{46, 2026}, []int{2464, 9477}, []int{5333, 9602},
			[]int{1332, 4930}, []int{1616, 463}, []int{9284, 7764}, []int{162, 8539}, []int{4296, 7591}, []int{629, 8804}, []int{604, 2043},
			[]int{919, 8459}, []int{637, 6310}, []int{3895, 8504}, []int{2048, 77}, []int{6028, 4756}, []int{1051, 5487}, []int{590, 2845},
			[]int{7178, 906}, []int{5243, 1110}, []int{1979, 5140}, []int{2548, 5732}, []int{2178, 9011}, []int{1869, 6098}, []int{918, 8517},
			[]int{2608, 702}, []int{3938, 4026}, []int{4683, 7977}, []int{4965, 7828}, []int{3773, 2779}, []int{6850, 9742}, []int{621, 3646},
			[]int{3169, 2434}, []int{1763, 7086}, []int{3020, 9153}, []int{3821, 287}, []int{7427, 7888}, []int{7052, 5063}, []int{649, 2613},
			[]int{420, 8495}, []int{388, 5406}, []int{5199, 4438}, []int{1425, 2058}, []int{4699, 2030}, []int{6477, 2221}, []int{8567, 477},
			[]int{8493, 7969}, []int{4762, 3262}, []int{3399, 432}, []int{2780, 1053}, []int{3351, 3835}, []int{5547, 7071}, []int{9243, 5430},
			[]int{728, 1159}, []int{855, 9305}, []int{3735, 8594}},
		[][]int{[]int{930, 7824}, []int{1683, 1180}, []int{1207, 76}, []int{2087, 3113}, []int{6101, 8065}, []int{2438, 6525},
			[]int{7984, 5702}, []int{6964, 9156}, []int{5661, 1016}, []int{7498, 8283}, []int{4154, 166}, []int{6950, 8261}, []int{8276, 3574},
			[]int{5640, 9831}, []int{8302, 6271}, []int{8007, 8268}, []int{8289, 9069}, []int{9658, 6081}, []int{5238, 5905}, []int{288, 5178},
			[]int{8622, 3550}, []int{2584, 4508}, []int{9643, 5997}, []int{4825, 3137}, []int{5924, 6938}, []int{2188, 1015}, []int{501, 3236},
			[]int{1517, 1681}, []int{4924, 1777}, []int{6525, 388}, []int{6109, 6984}, []int{9523, 3291}, []int{9727, 4373}, []int{4454, 3268},
			[]int{8701, 824}, []int{7493, 1089}, []int{1863, 9558}, []int{3023, 8776}, []int{9662, 1127}, []int{5499, 7891}, []int{3899, 9958},
			[]int{3682, 4497}, []int{9368, 6229}, []int{6288, 483}, []int{4293, 934}, []int{3446, 608}, []int{1941, 5986}, []int{2283, 2561},
			[]int{1902, 7062}, []int{1446, 5637}, []int{3885, 3937}, []int{1206, 7103}, []int{576, 7416}, []int{42, 771}, []int{2442, 7651},
			[]int{6121, 6461}, []int{5906, 7773}, []int{1100, 2985}, []int{3188, 8679}, []int{3855, 7500}, []int{5438, 1253}, []int{7380, 3930},
			[]int{9605, 9087}, []int{5855, 9837}, []int{3054, 5330}, []int{4011, 1541}, []int{8853, 5869}, []int{5763, 1491}, []int{8281, 7828},
			[]int{7386, 6460}, []int{9085, 753}, []int{6532, 8956}, []int{6345, 9982}, []int{2586, 3280}, []int{7320, 7052}, []int{7402, 1803},
			[]int{5768, 2799}, []int{4965, 8690}, []int{348, 1583}, []int{6720, 720}, []int{2701, 216}, []int{9667, 4659}, []int{853, 5627},
			[]int{4797, 4749}, []int{6610, 3195}, []int{5437, 7987}, []int{9754, 5441}, []int{2059, 5058}, []int{1057, 7945}, []int{6445, 6773},
			[]int{2740, 7938}, []int{9250, 2144}, []int{9697, 8578}, []int{4043, 4312}, []int{4941, 7168}, []int{3597, 7017}, []int{6589, 712},
			[]int{8112, 2751}, []int{3165, 6811}, []int{3539, 3021}, []int{3580, 4684}, []int{1733, 3097}, []int{862, 3668}, []int{5070, 941},
			[]int{2905, 398}, []int{8052, 2088}, []int{8440, 8300}, []int{38, 5643}, []int{4059, 1376}, []int{4933, 8701}, []int{4773, 6772},
			[]int{4678, 1912}, []int{9037, 2739}, []int{1, 4649}, []int{7613, 3725}, []int{3125, 9187}, []int{1510, 1527}, []int{1026, 1149},
			[]int{7242, 9912}, []int{4205, 6448}, []int{3041, 3902}, []int{8239, 698}, []int{2005, 1687}, []int{5830, 7357}, []int{7493, 1896},
			[]int{4435, 4240}, []int{6694, 1578}, []int{6411, 5741}, []int{2413, 2967}, []int{1398, 1378}, []int{8278, 9394}, []int{2947, 1632},
			[]int{9937, 604}, []int{1682, 8703}, []int{7519, 2032}, []int{1800, 5176}, []int{3177, 7864}, []int{2031, 6153}, []int{5066, 9653},
			[]int{5219, 933}, []int{4848, 6475}, []int{6230, 785}, []int{6895, 7218}, []int{4444, 424}, []int{3453, 4758}, []int{192, 5480},
			[]int{4711, 8264}, []int{318, 6739}, []int{267, 6316}, []int{9591, 2495}, []int{668, 2169}, []int{64, 6726}, []int{2866, 2636},
			[]int{194, 6215}, []int{2471, 1013}, []int{8924, 5139}, []int{2314, 2055}, []int{9117, 9985}, []int{3248, 9948}, []int{9898, 9626},
		}))

}
