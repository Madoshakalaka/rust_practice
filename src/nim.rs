const M0: u32 = 3880577560;
const M1: u32 = 38421560;
const M2: u32 = 163496;
const M3: u32 = 428;
const M4: u32 = 1;
const M: [u32; 5] = [M0, M1, M2, M3, M4];
const SIZE: u64 = 116417326800;
const INITIAL_STATE: u64 = 116417326800 - 1;

fn psn(coord: [u32; 5])->u32{
    M0*coord[0] + M1*coord[1] + M2*coord[2] + M3*coord[3] + coord[4]
}

fn crd(psn:u32)-> [u32; 5]{
    let mut crd: [u32; 5] = [0,0,0,0,0];

    let mut r: u32 = psn;
    let mut d: u32;
    for j in 0u8..5 {
        d = r / M[j as usize];
        r = r - d* M[j as usize];
        crd[j as usize] = d
    }
    crd

}


//       for j in tqdm(range(self.size - 1)): # nothing reaches last state
//         if j not in self.win_indexes: # loss, so find all psns that reach j
//           cj = self.crd(j)
//           loss_updates += 1
//           #print(cj,'loses, find all wins that reach this')
//           for x in range(len(cj)):
//             cjcopy = copy(cj)
//             for t in range(1+cj[x], 1+self.dim[x]):
//               cjcopy[x] = t
//               pjc = self.psn(cjcopy)
//               # self.wins[pjc], self.winmove[pjc] = True, j
//               self.win_indexes.add(pjc)
//               win_updates += 1