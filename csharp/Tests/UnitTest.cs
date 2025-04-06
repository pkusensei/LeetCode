using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 2, 3 }, "[1,2]")]
    [DataRow(new[] { 1, 2, 4, 8 }, "[1,2,4,8]")]
    public void TestMethod1(int[] nums, string exp)
    {
        Assert.AreEqual(exp, sol.LargestDivisibleSubset(nums).Print());
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}