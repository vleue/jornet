import { PureComponent } from "react";
import { Col, Container, Row, Table } from "react-bootstrap";
import { useParams } from "react-router-dom";


type Score = {
    score: number,
    meta?: string,
    timestamp: string,
    player: string,
}

type LeaderboardProps = {
    leaderboardId?: string,
    refresh: number,
};
type LeaderboardState = {
    scores: Score[],
};


class LeaderboardInner extends PureComponent<LeaderboardProps, LeaderboardState> {
    state: LeaderboardState = {
        scores: []
    };
    componentDidMount() {
        fetch("/api/v1/scores/" + this.props.leaderboardId)
            .then(response => response.json())
            .then(data => {
                this.setState({ scores: data });
            });
    }

    componentDidUpdate(prevProps: LeaderboardProps) {
        if (this.props.refresh !== prevProps.refresh) {
            fetch("/api/v1/scores/" + this.props.leaderboardId)
                .then(response => response.json())
                .then(data => {
                    this.setState({ scores: data });
                });
        }
    }

    render() {
        return (
            <Container fluid="lg">
                <Row>
                    <Col>&nbsp;</Col>
                </Row>
                <Row>
                    <Col>
                        <Table striped bordered hover>
                            <thead>
                                <tr>
                                    <th>Score</th>
                                    <th>Player</th>
                                    <th>Timestamp</th>
                                    <th>Meta</th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    this.state.scores.map((score, index) => {
                                        return <tr key={index}>
                                            <td>{score.score}</td>
                                            <td>{score.player}</td>
                                            <td>{score.timestamp}</td>
                                            <td>{score.meta}</td>
                                        </tr>
                                    })
                                }
                            </tbody>
                        </Table>
                    </Col>
                </Row>
            </Container >
        );
    }
}

export default function Leaderboard(props: LeaderboardProps) {
    let params = useParams();
    if (props.leaderboardId === undefined) {
        return <LeaderboardInner {...props} leaderboardId={params.leaderboardId} />;
    } else {
        return <LeaderboardInner {...props} />;
    }
};
