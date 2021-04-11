#include <vector>
#include <iostream>

using std::cout;
using std::endl;
using std::vector;

vector<int> twoSum(vector<int> &nums, int target)
{
    vector<int>::size_type first = 0;
    vector<int>::size_type second;
    for (; first < nums.size(); first += 1)
    {
        for (second = first + 1; second < nums.size(); second += 1)
        {
            if ((nums[first] + nums[second]) == target)
            {
                vector<int> position{static_cast<int>(first), static_cast<int>(second)};
                return position;
            }
        }
    }
    vector<int> position{0, 1};
    return position;
}

int main()
{
    vector<int> nums{2, 8, 13, 15};
    int target = 28;

    vector<int> result = twoSum(nums, target);
    cout << result[0] << endl;
    cout << result[1] << endl;

    if (result[0] == 2 && result[1] == 3)
    {
        cout << "通过" << endl;
    }

    return 0;
}
