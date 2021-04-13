# basin city surveillance
source: https://nordic.icpc.io/ncpc2014/

oj: https://open.kattis.com/problems/basincity

Time Complexity: O(3^k);

I'm not smart, I copied from an [answer](https://github.com/AlexDevyatov/ACM-Contests-Data/blob/7cac551ab9622bf01925a5e4c735bc3e0fc0a471/NCPC/2014/ncpc2014/basincity/submissions/accepted/marek.cpp).

Obvioulsy, for any intersection, it or its one neighbor must be included in an optimal solution.

Furthermore, for any intersection, there will either be an optimal solution including it, or max(two, neighbours_count) of its neighbours, why?

Let's take intersection ***a*** for example, ***b***, ***c***, ***d***, ***e*** are its neighbor. ***a***, ***b***, ***c***, ***d*** is not placed a drone, ***e*** is placed a drone. If one of ***b***, ***c***, ***d*** only take a for its neighbor, we can put a drone on it. Or not we can easily move a drone from ***b***/***c***/***d***'s neighbor to ***b***/***c***/***d***. Therefore, for any intersection, if there's an optimal solution, the drone is included, or at least max(2, neighbors_count) of its neighbors are in included.

More details are in book [principles of algorithmic problem solving](https://www.csc.kth.se/~jsannemo/slask/main.pdf) Page 130:

