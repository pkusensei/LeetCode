using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("1-2--3--4-5--6--7", "[1,2,5,3,4,6,7]")]
    [DataRow("1-2--3---4-5--6---7", "[1,2,5,3,null,6,null,4,null,7]")]
    [DataRow("1-401--349---90--88", "[1,401,null,349,88,90]")]
    public void TestMethod1(string s, string exp)
    {
        var a = sol.RecoverFromPreorder(s);
        Assert.AreEqual(exp, a.ToString());
    }


    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }
}