import Files from './bits/Files'
import Ranks from './bits/Ranks'
import './board.css'

const Board = () => {
    
    const ranks = Array(8).fill().map((_,i)=>8-i)
    const files = Array(8).fill().map((_,i)=>i)

    const getClassName = (i,j) =>{
        return (i+j)%2 === 0 ? 'tile--light' : 'tile--dark'

    }
    return <div className='Board'>

        <Ranks ranks={ranks}/>
        <div className='tiles'>
        {ranks.map((rank,i) => 
                files.map((file,j) => 
                    <div key={file+'-'+rank} className={getClassName(i,j)}>
                    </div>
                ))}
        </div>
        <Files files={files}/>

    </div>
}

export default Board